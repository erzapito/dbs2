import { shallowMount } from '@vue/test-utils';
import Paginator from '@/components/paginator.vue';

describe('Paginator.vue', () => {
    it('renders page 1', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 1,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.html()).toMatch('<div class=\"paginator\"><!----> <!----> <!----> ' +
            '<span class=\"paginator-cell\">1</span> <span class=\"paginator-cell\">' +
            '<a href=\"javascript:void(0)\">2</a></span> <span>...</span> ' +
            '<span class=\"paginator-cell\"><a href=\"javascript:void(0)\">10</a></span></div>');
    });
    it('renders page 2', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 2,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.html()).toMatch('<div class=\"paginator\"><span class=\"paginator-cell\">'+
        '<a href=\"javascript:void(0)\">1</a></span> <!----> <!----> '+
        '<span class=\"paginator-cell\">2</span> <span class=\"paginator-cell\">'+
        '<a href=\"javascript:void(0)\">3</a></span> <span>...</span> '+
        '<span class=\"paginator-cell\"><a href=\"javascript:void(0)\">10</a></span></div>');
    });
    it('renders page 3', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 3,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.html()).toMatch('<div class=\"paginator\"><span class=\"paginator-cell\">'+
        '<a href=\"javascript:void(0)\">1</a></span> <!----> <span class=\"paginator-cell\">'+
        '<a href=\"javascript:void(0)\">2</a></span> <span class=\"paginator-cell\">3</span> '+
        '<span class=\"paginator-cell\"><a href=\"javascript:void(0)\">4</a></span> '+
        '<span>...</span> <span class=\"paginator-cell\"><a href=\"javascript:void(0)\">10</a>'+
        '</span></div>');
    });
    it('renders page 4', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 4,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.html()).toMatch('<div class=\"paginator\"><span class=\"paginator-cell\">'+
        '<a href=\"javascript:void(0)\">1</a></span> <span>...</span> '+
        '<span class=\"paginator-cell\"><a href=\"javascript:void(0)\">3</a></span> '+
        '<span class=\"paginator-cell\">4</span> <span class=\"paginator-cell\">'+
        '<a href=\"javascript:void(0)\">5</a></span> <span>...</span> '+
        '<span class=\"paginator-cell\"><a href=\"javascript:void(0)\">10</a></span></div>');
    });
    it('renders page 8', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 8,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.html()).toMatch('<div class=\"paginator\"><span class=\"paginator-cell\">'+
        '<a href=\"javascript:void(0)\">1</a></span> <span>...</span> '+
        '<span class=\"paginator-cell\"><a href=\"javascript:void(0)\">7</a></span> '+
        '<span class=\"paginator-cell\">8</span> <span class=\"paginator-cell\">'+
        '<a href=\"javascript:void(0)\">9</a></span> <!----> <span class=\"paginator-cell\">'+
        '<a href=\"javascript:void(0)\">10</a></span></div>');
    });
    it('renders page 9', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 9,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.html()).toMatch('<div class=\"paginator\"><span class=\"paginator-cell\">'+
        '<a href=\"javascript:void(0)\">1</a></span> <span>...</span> '+
        '<span class=\"paginator-cell\"><a href=\"javascript:void(0)\">8</a></span> '+
        '<span class=\"paginator-cell\">9</span> <!----> <!----> <span class=\"paginator-cell\">'+
        '<a href=\"javascript:void(0)\">10</a></span></div>');
    });
    it('renders page 10', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 10,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.html()).toMatch('<div class="paginator"><span class="paginator-cell">'+
        '<a href="javascript:void(0)">1</a></span> <span>...</span> <span class="paginator-cell">'+
        '<a href="javascript:void(0)">9</a></span> <span class="paginator-cell">10</span> <!----> '+
        '<!----> <!----></div>');
    });

    it('renders 0 items', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 10,
                pageSize : 10,
                totalItems : 0,
            },
        });
        expect(wrapper.html()).toBeUndefined();
    });

});
