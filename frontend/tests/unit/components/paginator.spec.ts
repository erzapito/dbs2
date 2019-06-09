import { shallowMount } from '@vue/test-utils';
import Paginator from '@/components/paginator.vue';

describe('Paginator.vue', () => {
    it('renders page 1', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 1,
            },
        });
        expect(wrapper.text()).toMatch('1 2 ... 10');
    });
    it('renders page 2', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 2,
            },
        });
        expect(wrapper.text()).toMatch('1   2 3 ... 10');
    });
    it('renders page 3', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 3,
            },
        });
        expect(wrapper.text()).toMatch('1  2 3 4 ... 10');
    });
    it('renders page 4', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 4,
            },
        });
        expect(wrapper.text()).toMatch('1 ... 3 4 5 ... 10');
    });
    it('renders page 8', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 8,
            },
        });
        expect(wrapper.text()).toMatch('1 ... 7 8 9  10');
    });
    it('renders page 9', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 9,
            },
        });
        expect(wrapper.text()).toMatch('1 ... 8 9   10');
    });
    it('renders page 10', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 10,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.text()).toMatch('1 ... 9 10');
    });
});
