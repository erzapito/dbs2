import { shallowMount } from '@vue/test-utils';
import EditField from '@/components/edit-field.vue';

describe('edit-field.vue', () => {

    it('renders', (done) => {
        const item = {
            'test' : 'T',
        };
        const wrapper = shallowMount(EditField,{
            propsData: {
                'label' : 'TEST LABEL',
                'field' : item.test
            }
        });
        expect(wrapper.findAll('label').exists()).toBe(true);
        expect(wrapper.findAll('input').exists()).toBe(true);
        done();
    });

});
